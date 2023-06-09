use crate::errors::*;
use crate::kernel::actor::{Actor, ActorIdentifier};
use crate::kernel::call_frame::CallFrameUpdate;
use crate::kernel::kernel_api::KernelModuleApi;
use crate::kernel::module::KernelModule;
use crate::system::node::RENodeInit;
use crate::system::node::RENodeModuleInit;
use crate::types::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::math::Decimal;
use sbor::rust::collections::*;
use sbor::rust::fmt::Debug;

//===================================================================================
// Note: ExecutionTrace must not produce any error or transactional side effect!
//===================================================================================

#[derive(Debug, Clone)]
pub struct ExecutionTraceModule {
    /// Maximum depth up to which kernel calls are being traced.
    max_kernel_call_depth_traced: usize,

    /// Current transaction index
    current_instruction_index: usize,

    /// Current kernel calls depth. Note that this doesn't necessarily correspond to the
    /// call frame depth, as there can be nested kernel calls within a single call frame
    /// (e.g. lock_substate call inside drop_node).
    current_kernel_call_depth: usize,

    /// A stack of traced kernel call inputs, their origin, and the instruction index.
    traced_kernel_call_inputs_stack: Vec<(ResourceSummary, Origin, usize)>,

    /// A mapping of complete KernelCallTrace stacks (\w both inputs and outputs), indexed by depth.
    kernel_call_traces_stacks: IndexMap<usize, Vec<ExecutionTrace>>,

    /// Vault operations: (Caller, Vault ID, operation, instruction index)
    vault_ops: Vec<(TraceActor, ObjectId, VaultOp, usize)>,
}

impl ExecutionTraceModule {
    pub fn update_instruction_index(&mut self, new_index: usize) {
        self.current_instruction_index = new_index;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct ResourceChange {
    pub resource_address: ResourceAddress,
    pub node_id: RENodeId,
    pub vault_id: ObjectId,
    pub amount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum WorktopChange {
    Take(ResourceSpecifier),
    Put(ResourceSpecifier),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum ResourceSpecifier {
    Amount(ResourceAddress, Decimal),
    Ids(ResourceAddress, BTreeSet<NonFungibleLocalId>),
}

impl From<&BucketSnapshot> for ResourceSpecifier {
    fn from(value: &BucketSnapshot) -> Self {
        match value {
            BucketSnapshot::Fungible {
                resource_address,
                liquid,
                ..
            } => Self::Amount(*resource_address, *liquid),
            BucketSnapshot::NonFungible {
                resource_address,
                liquid,
                ..
            } => Self::Ids(*resource_address, liquid.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VaultOp {
    Create(Decimal),               // TODO: add trace of vault creation
    Put(ResourceAddress, Decimal), // TODO: add non-fungible support
    Take(ResourceAddress, Decimal),
    LockFee,
}

#[derive(Clone, Debug, PartialEq, Eq, ScryptoSbor)]
pub enum BucketSnapshot {
    Fungible {
        resource_address: ResourceAddress,
        resource_type: ResourceType,
        liquid: Decimal,
    },
    NonFungible {
        resource_address: ResourceAddress,
        resource_type: ResourceType,
        liquid: BTreeSet<NonFungibleLocalId>,
    },
}

impl BucketSnapshot {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            BucketSnapshot::Fungible {
                resource_address, ..
            } => resource_address.clone(),
            BucketSnapshot::NonFungible {
                resource_address, ..
            } => resource_address.clone(),
        }
    }
    pub fn amount(&self) -> Decimal {
        match self {
            BucketSnapshot::Fungible { liquid, .. } => liquid.clone(),
            BucketSnapshot::NonFungible { liquid, .. } => liquid.len().into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ScryptoSbor)]
pub enum ProofSnapshot {
    Fungible {
        resource_address: ResourceAddress,
        resource_type: ResourceType,
        restricted: bool,
        total_locked: Decimal,
    },
    NonFungible {
        resource_address: ResourceAddress,
        resource_type: ResourceType,
        restricted: bool,
        total_locked: BTreeSet<NonFungibleLocalId>,
    },
}

impl ProofSnapshot {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            ProofSnapshot::Fungible {
                resource_address, ..
            } => resource_address.clone(),
            ProofSnapshot::NonFungible {
                resource_address, ..
            } => resource_address.clone(),
        }
    }
    pub fn amount(&self) -> Decimal {
        match self {
            ProofSnapshot::Fungible { total_locked, .. } => total_locked.clone(),
            ProofSnapshot::NonFungible { total_locked, .. } => total_locked.len().into(),
        }
    }
}

#[derive(Debug, Clone, ScryptoSbor)]
pub struct ResourceSummary {
    pub buckets: IndexMap<ObjectId, BucketSnapshot>,
    pub proofs: IndexMap<ObjectId, ProofSnapshot>,
}

// TODO: Clean up
#[derive(Debug, Clone, ScryptoSbor)]
pub enum TraceActor {
    Root,
    Actor(Actor),
}

#[derive(Debug, Clone, ScryptoSbor)]
pub struct ExecutionTrace {
    pub origin: Origin,
    pub kernel_call_depth: usize,
    pub current_frame_actor: TraceActor,
    pub current_frame_depth: usize,
    pub instruction_index: usize,
    pub input: ResourceSummary,
    pub output: ResourceSummary,
    pub children: Vec<ExecutionTrace>,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub enum Origin {
    ScryptoFunction(FnIdentifier),
    ScryptoMethod(FnIdentifier),
    CreateNode,
    DropNode,
}

impl ExecutionTrace {
    pub fn worktop_changes(
        &self,
        worktop_changes_aggregator: &mut IndexMap<usize, Vec<WorktopChange>>,
    ) {
        if let Origin::ScryptoMethod(fn_identifier) = &self.origin {
            if fn_identifier.blueprint_name == WORKTOP_BLUEPRINT
                && fn_identifier.package_address == RESOURCE_MANAGER_PACKAGE
            {
                if fn_identifier.ident == WORKTOP_PUT_IDENT {
                    for (_, bucket_snapshot) in self.input.buckets.iter() {
                        worktop_changes_aggregator
                            .entry(self.instruction_index)
                            .or_default()
                            .push(WorktopChange::Put(bucket_snapshot.into()))
                    }
                } else if fn_identifier.ident == WORKTOP_TAKE_IDENT
                    || fn_identifier.ident == WORKTOP_TAKE_ALL_IDENT
                    || fn_identifier.ident == WORKTOP_TAKE_NON_FUNGIBLES_IDENT
                    || fn_identifier.ident == WORKTOP_DRAIN_IDENT
                {
                    for (_, bucket_snapshot) in self.output.buckets.iter() {
                        worktop_changes_aggregator
                            .entry(self.instruction_index)
                            .or_default()
                            .push(WorktopChange::Take(bucket_snapshot.into()))
                    }
                }
            }
        }

        // Aggregate the worktop changes for all children traces
        for child in self.children.iter() {
            child.worktop_changes(worktop_changes_aggregator)
        }
    }
}

impl ResourceSummary {
    pub fn default() -> Self {
        Self {
            buckets: index_map_new(),
            proofs: index_map_new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty() && self.proofs.is_empty()
    }

    pub fn from_call_frame_update<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        call_frame_update: &CallFrameUpdate,
    ) -> Self {
        let mut buckets = index_map_new();
        let mut proofs = index_map_new();
        for node_id in &call_frame_update.nodes_to_move {
            match &node_id {
                RENodeId::Object(object_id) => {
                    if let Some(x) = api.kernel_read_bucket(*object_id) {
                        buckets.insert(*object_id, x);
                    }
                    if let Some(x) = api.kernel_read_proof(*object_id) {
                        proofs.insert(*object_id, x);
                    }
                }
                _ => {}
            }
        }
        Self { buckets, proofs }
    }

    pub fn from_node_id<Y: KernelModuleApi<RuntimeError>>(api: &mut Y, node_id: &RENodeId) -> Self {
        let mut buckets = index_map_new();
        let mut proofs = index_map_new();
        match node_id {
            RENodeId::Object(object_id) => {
                if let Some(x) = api.kernel_read_bucket(*object_id) {
                    buckets.insert(*object_id, x);
                }
                if let Some(x) = api.kernel_read_proof(*object_id) {
                    proofs.insert(*object_id, x);
                }
            }
            _ => {}
        }
        Self { buckets, proofs }
    }
}

impl KernelModule for ExecutionTraceModule {
    fn before_create_node<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        _node_id: &RENodeId,
        _node_init: &RENodeInit,
        _node_module_init: &BTreeMap<NodeModuleId, RENodeModuleInit>,
    ) -> Result<(), RuntimeError> {
        api.kernel_get_module_state()
            .execution_trace
            .handle_before_create_node();
        Ok(())
    }

    fn after_create_node<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        node_id: &RENodeId,
    ) -> Result<(), RuntimeError> {
        let current_actor = api.kernel_get_current_actor();
        let current_depth = api.kernel_get_current_depth();
        let resource_summary = ResourceSummary::from_node_id(api, node_id);
        api.kernel_get_module_state()
            .execution_trace
            .handle_after_create_node(current_actor, current_depth, resource_summary);
        Ok(())
    }

    fn before_drop_node<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        node_id: &RENodeId,
    ) -> Result<(), RuntimeError> {
        let resource_summary = ResourceSummary::from_node_id(api, node_id);
        api.kernel_get_module_state()
            .execution_trace
            .handle_before_drop_node(resource_summary);
        Ok(())
    }

    fn after_drop_node<Y: KernelModuleApi<RuntimeError>>(api: &mut Y) -> Result<(), RuntimeError> {
        let current_actor = api.kernel_get_current_actor();
        let current_depth = api.kernel_get_current_depth();
        api.kernel_get_module_state()
            .execution_trace
            .handle_after_drop_node(current_actor, current_depth);
        Ok(())
    }

    fn before_push_frame<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        callee: &Option<Actor>,
        update: &mut CallFrameUpdate,
        _args: &IndexedScryptoValue,
    ) -> Result<(), RuntimeError> {
        let current_actor = api.kernel_get_current_actor();
        let resource_summary = ResourceSummary::from_call_frame_update(api, update);
        api.kernel_get_module_state()
            .execution_trace
            .handle_before_push_frame(current_actor, callee, resource_summary);
        Ok(())
    }

    fn on_execution_finish<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        caller: &Option<Actor>,
        update: &CallFrameUpdate,
    ) -> Result<(), RuntimeError> {
        let current_actor = api.kernel_get_current_actor();
        let current_depth = api.kernel_get_current_depth();
        let resource_summary = ResourceSummary::from_call_frame_update(api, update);
        api.kernel_get_module_state()
            .execution_trace
            .handle_on_execution_finish(current_actor, current_depth, caller, resource_summary);
        Ok(())
    }
}

impl ExecutionTraceModule {
    pub fn new(max_kernel_call_depth_traced: usize) -> ExecutionTraceModule {
        Self {
            max_kernel_call_depth_traced,
            current_instruction_index: 0,
            current_kernel_call_depth: 0,
            traced_kernel_call_inputs_stack: vec![],
            kernel_call_traces_stacks: index_map_new(),
            vault_ops: Vec::new(),
        }
    }

    fn handle_before_create_node(&mut self) {
        if self.current_kernel_call_depth <= self.max_kernel_call_depth_traced {
            let instruction_index = self.instruction_index();

            let traced_input = (
                ResourceSummary::default(),
                Origin::CreateNode,
                instruction_index,
            );
            self.traced_kernel_call_inputs_stack.push(traced_input);
        }

        self.current_kernel_call_depth += 1;
    }

    fn handle_after_create_node(
        &mut self,
        current_actor: Option<Actor>,
        current_depth: usize,
        resource_summary: ResourceSummary,
    ) {
        // Important to always update the counter (even if we're over the depth limit).
        self.current_kernel_call_depth -= 1;

        if self.current_kernel_call_depth > self.max_kernel_call_depth_traced {
            // Nothing to trace at this depth, exit.
            return;
        }

        let current_actor = current_actor
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        self.finalize_kernel_call_trace(resource_summary, current_actor, current_depth)
    }

    fn handle_before_drop_node(&mut self, resource_summary: ResourceSummary) {
        if self.current_kernel_call_depth <= self.max_kernel_call_depth_traced {
            let instruction_index = self.instruction_index();

            let traced_input = (resource_summary, Origin::DropNode, instruction_index);
            self.traced_kernel_call_inputs_stack.push(traced_input);
        }

        self.current_kernel_call_depth += 1;
    }

    fn handle_after_drop_node(&mut self, current_actor: Option<Actor>, current_depth: usize) {
        // Important to always update the counter (even if we're over the depth limit).
        self.current_kernel_call_depth -= 1;

        if self.current_kernel_call_depth > self.max_kernel_call_depth_traced {
            // Nothing to trace at this depth, exit.
            return;
        }

        let traced_output = ResourceSummary::default();

        let current_actor = current_actor
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        self.finalize_kernel_call_trace(traced_output, current_actor, current_depth)
    }

    fn handle_before_push_frame(
        &mut self,
        current_actor: Option<Actor>,
        callee: &Option<Actor>,
        resource_summary: ResourceSummary,
    ) {
        if self.current_kernel_call_depth <= self.max_kernel_call_depth_traced {
            let origin = match &callee {
                Some(Actor {
                    fn_identifier: identifier,
                    identifier: receiver,
                }) => match receiver {
                    ActorIdentifier::Method(..) => Origin::ScryptoMethod(identifier.clone()),
                    ActorIdentifier::Function(..) => Origin::ScryptoFunction(identifier.clone()),
                },
                _ => panic!("Should not get here."),
            };

            let instruction_index = self.instruction_index();

            self.traced_kernel_call_inputs_stack.push((
                resource_summary.clone(),
                origin,
                instruction_index,
            ));
        }

        self.current_kernel_call_depth += 1;

        match &callee {
            Some(Actor {
                fn_identifier:
                    FnIdentifier {
                        package_address,
                        blueprint_name,
                        ident,
                    },
                identifier:
                    ActorIdentifier::Method(MethodIdentifier(RENodeId::Object(vault_id), ..)),
            }) if package_address.eq(&RESOURCE_MANAGER_PACKAGE)
                && blueprint_name.eq(VAULT_BLUEPRINT)
                && ident.eq(VAULT_PUT_IDENT) =>
            {
                self.handle_vault_put_input(&resource_summary, &current_actor, vault_id)
            }
            Some(Actor {
                fn_identifier:
                    FnIdentifier {
                        package_address,
                        blueprint_name,
                        ident,
                    },
                identifier:
                    ActorIdentifier::Method(MethodIdentifier(RENodeId::Object(vault_id), ..)),
            }) if package_address.eq(&RESOURCE_MANAGER_PACKAGE)
                && blueprint_name.eq(VAULT_BLUEPRINT)
                && ident.eq(VAULT_LOCK_FEE_IDENT) =>
            {
                self.handle_vault_lock_fee_input(&current_actor, vault_id)
            }
            _ => {}
        }
    }

    fn handle_on_execution_finish(
        &mut self,
        current_actor: Option<Actor>,
        current_depth: usize,
        caller: &Option<Actor>,
        resource_summary: ResourceSummary,
    ) {
        match &current_actor {
            Some(Actor {
                fn_identifier:
                    FnIdentifier {
                        package_address,
                        blueprint_name,
                        ident,
                    },
                identifier:
                    ActorIdentifier::Method(MethodIdentifier(RENodeId::Object(vault_id), ..)),
            }) if package_address.eq(&RESOURCE_MANAGER_PACKAGE)
                && blueprint_name.eq(VAULT_BLUEPRINT)
                && ident.eq(VAULT_TAKE_IDENT) =>
            {
                self.handle_vault_take_output(&resource_summary, caller, vault_id)
            }
            _ => {}
        }

        // Important to always update the counter (even if we're over the depth limit).
        self.current_kernel_call_depth -= 1;

        if self.current_kernel_call_depth > self.max_kernel_call_depth_traced {
            // Nothing to trace at this depth, exit.
            return;
        }

        let current_actor = current_actor
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        self.finalize_kernel_call_trace(resource_summary, current_actor, current_depth)
    }

    fn finalize_kernel_call_trace(
        &mut self,
        traced_output: ResourceSummary,
        current_actor: TraceActor,
        current_depth: usize,
    ) {
        let child_traces = self
            .kernel_call_traces_stacks
            .remove(&(self.current_kernel_call_depth + 1))
            .unwrap_or(vec![]);

        let (traced_input, origin, instruction_index) = self
            .traced_kernel_call_inputs_stack
            .pop()
            .expect("kernel call input stack underflow");

        // Only include the trace if:
        // * there's a non-empty traced input or output
        // * OR there are any child traces: they need a parent regardless of whether it traces any inputs/outputs.
        //   At some depth (up to the tracing limit) there must have been at least one traced input/output
        //   so we need to include the full path up to the root.
        if !traced_input.is_empty() || !traced_output.is_empty() || !child_traces.is_empty() {
            let trace = ExecutionTrace {
                origin,
                kernel_call_depth: self.current_kernel_call_depth,
                current_frame_actor: current_actor,
                current_frame_depth: current_depth,
                instruction_index,
                input: traced_input,
                output: traced_output,
                children: child_traces,
            };

            let siblings = self
                .kernel_call_traces_stacks
                .entry(self.current_kernel_call_depth)
                .or_insert(vec![]);
            siblings.push(trace);
        }
    }

    pub fn collect_traces(
        mut self,
    ) -> (
        Vec<ExecutionTrace>,
        Vec<(TraceActor, ObjectId, VaultOp, usize)>,
    ) {
        let mut execution_traces = Vec::new();
        for (_, traces) in self.kernel_call_traces_stacks.drain(..) {
            execution_traces.extend(traces);
        }

        (execution_traces, self.vault_ops)
    }

    fn instruction_index(&self) -> usize {
        self.current_instruction_index
    }

    fn handle_vault_put_input<'s>(
        &mut self,
        resource_summary: &ResourceSummary,
        caller: &Option<Actor>,
        vault_id: &ObjectId,
    ) {
        let actor = caller
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        for (_, resource) in &resource_summary.buckets {
            self.vault_ops.push((
                actor.clone(),
                vault_id.clone(),
                VaultOp::Put(resource.resource_address(), resource.amount()),
                self.instruction_index(),
            ));
        }
    }

    fn handle_vault_lock_fee_input<'s>(&mut self, caller: &Option<Actor>, vault_id: &ObjectId) {
        let actor = caller
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        self.vault_ops.push((
            actor,
            vault_id.clone(),
            VaultOp::LockFee,
            self.instruction_index(),
        ));
    }

    fn handle_vault_take_output<'s>(
        &mut self,
        resource_summary: &ResourceSummary,
        caller: &Option<Actor>,
        vault_id: &ObjectId,
    ) {
        let actor = caller
            .clone()
            .map(|a| TraceActor::Actor(a))
            .unwrap_or(TraceActor::Root);
        for (_, resource) in &resource_summary.buckets {
            self.vault_ops.push((
                actor.clone(),
                vault_id.clone(),
                VaultOp::Take(resource.resource_address(), resource.amount()),
                self.instruction_index(),
            ));
        }
    }
}

pub fn calculate_resource_changes(
    vault_ops: Vec<(TraceActor, ObjectId, VaultOp, usize)>,
    fee_payments: &IndexMap<ObjectId, Decimal>,
    is_commit_success: bool,
) -> IndexMap<usize, Vec<ResourceChange>> {
    // FIXME: revert non-fee operations if `is_commit_success == false`

    let mut vault_changes =
        index_map_new::<usize, IndexMap<RENodeId, IndexMap<ObjectId, (ResourceAddress, Decimal)>>>(
        );
    let mut vault_locked_by = index_map_new::<ObjectId, RENodeId>();
    for (actor, vault_id, vault_op, instruction_index) in vault_ops {
        if let TraceActor::Actor(Actor {
            identifier: ActorIdentifier::Method(MethodIdentifier(node_id, ..)),
            ..
        }) = actor
        {
            match vault_op {
                VaultOp::Create(_) => todo!("Not supported yet!"),
                VaultOp::Put(resource_address, amount) => {
                    vault_changes
                        .entry(instruction_index)
                        .or_default()
                        .entry(node_id)
                        .or_default()
                        .entry(vault_id)
                        .or_insert((resource_address, Decimal::zero()))
                        .1 += amount;
                }
                VaultOp::Take(resource_address, amount) => {
                    vault_changes
                        .entry(instruction_index)
                        .or_default()
                        .entry(node_id)
                        .or_default()
                        .entry(vault_id)
                        .or_insert((resource_address, Decimal::zero()))
                        .1 -= amount;
                }
                VaultOp::LockFee => {
                    vault_changes
                        .entry(instruction_index)
                        .or_default()
                        .entry(node_id)
                        .or_default()
                        .entry(vault_id)
                        .or_insert((RADIX_TOKEN, Decimal::zero()))
                        .1 -= 0;

                    // Hack: Additional check to avoid second `lock_fee` attempts (runtime failure) from
                    // polluting the `vault_locked_by` index.
                    if !vault_locked_by.contains_key(&vault_id) {
                        vault_locked_by.insert(vault_id, node_id);
                    }
                }
            }
        }
    }

    let mut resource_changes = index_map_new::<usize, Vec<ResourceChange>>();
    for (instruction_index, instruction_resource_changes) in vault_changes {
        for (node_id, map) in instruction_resource_changes {
            for (vault_id, (resource_address, delta)) in map {
                // Amount = put/take amount - fee_amount
                let fee_amount = fee_payments.get(&vault_id).cloned().unwrap_or_default();
                let amount = if is_commit_success {
                    delta
                } else {
                    Decimal::zero()
                } - fee_amount;

                // Add a resource change log if non-zero
                if !amount.is_zero() {
                    resource_changes
                        .entry(instruction_index)
                        .or_default()
                        .push(ResourceChange {
                            resource_address,
                            node_id,
                            vault_id,
                            amount,
                        });
                }
            }
        }
    }

    resource_changes
}
