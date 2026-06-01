use serde_json::Value;

mod communication;
mod enterprise;
mod finalizers;
mod knowledge;
mod productivity;
mod workflow;

use communication::communication_manifest_modules;
use enterprise::enterprise_manifest_modules;
use finalizers::final_manifest_modules;
use knowledge::knowledge_manifest_modules;
use productivity::productivity_manifest_modules;
use workflow::workflow_manifest_modules;

pub(in crate::app) fn manifest_modules() -> Vec<Value> {
    let mut modules = Vec::new();
    modules.extend(workflow_manifest_modules());
    modules.extend(communication_manifest_modules());
    modules.extend(knowledge_manifest_modules());
    modules.extend(productivity_manifest_modules());
    modules.extend(enterprise_manifest_modules());
    modules.extend(final_manifest_modules());
    modules
}
