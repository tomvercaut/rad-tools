use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

use crate::model::{DicomFile, SopClass};
use tracing::trace;

/// Represents a view model item that contains information about a DICOM file.
#[derive(Clone, Debug, Default)]
pub struct ViewModelItem {
    patient_id: String,
    patient_name: String,
    sop: SopClass,
    plan_name: String,
    plan_label: String,
    dose_grids: usize,
}

pub fn build_model(dfs: &[DicomFile]) -> Vec<ViewModelItem> {
    let mut view = vec![];
    for df in dfs {
        if let DicomFile::RTPlan(plan) = df {
            view.push(ViewModelItem {
                patient_id: plan.patient_id.clone(),
                patient_name: plan.patient_name.clone(),
                sop: plan.sop.clone(),
                plan_name: plan.plan_name.clone(),
                plan_label: plan.plan_label.clone(),
                dose_grids: 0,
            });
        }
    }
    for df in dfs {
        if let DicomFile::RTDose(dose) = df {
            for rsc in &dose.referenced_rtplan_sequence {
                for v in &mut view {
                    if rsc.ref_class_uid == v.sop.class_uid
                        && rsc.ref_instance_uid == v.sop.instance_uid
                    {
                        v.dose_grids += 1;
                    }
                }
            }
        }
    }
    view
}

pub fn build_view(items: &[ViewModelItem]) -> Table {
    trace!("Building tabluar view.");
    let hdr = ["Patient ID", "Patient Name", "Plan Name", "Plan Label"];

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(hdr);
    for item in items {
        table.add_row([
            &item.patient_id,
            &item.patient_name,
            &item.plan_name,
            &item.plan_label,
        ]);
    }
    table
}
