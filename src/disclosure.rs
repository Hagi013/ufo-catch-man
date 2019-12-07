use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Disclosure {
    metadata: DisclosureContent,
    results: Vec<DisclosureResult>,
}

impl Disclosure {
    pub fn get_ufo_doc_list(&self) -> Vec<DisclosureResult> {
        self.results
            .iter()
            .filter(|&dis| { dis.filter_ufo() })
            .cloned()
            .collect()
    }
}

pub fn parse_to_entity(body: String) -> Result<Disclosure, Box<dyn std::error::Error>> {
    let entity = serde_json::from_str(&body).or(Err("Parse disclosure is failed.".to_string()))?;
    Ok(entity)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisclosureContent {
    title: String,
    parameter: DisclosureParameter,
    resultset: DisclosureResultSet,
    #[serde(rename = "processDateTime")]
    process_date_time: String,
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisclosureParameter {
    date: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisclosureResultSet {
    count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisclosureResult {
    #[serde(rename = "seqNumber")]
    seq_number: u32,
    #[serde(rename = "docID")]
    pub doc_id: String,
    #[serde(rename = "edinetCode")]
    edinet_code: Option<String>,
    #[serde(rename = "secCode")]
    pub sec_code: Option<String>,
    #[serde(rename = "JCN")]
    jcn: Option<String>,
    #[serde(rename = "filerName")]
    pub filer_name: Option<String>,
    #[serde(rename = "fundCode")]
    fund_code: Option<String>,
    #[serde(rename = "ordinanceCode")]
    ordinance_code: Option<String>,
    #[serde(rename = "formCode")]
    form_code: Option<String>,
    #[serde(rename = "docTypeCode")]
    doc_type_code: Option<String>,
    #[serde(rename = "periodStart")]
    pub period_start: Option<String>,
    #[serde(rename = "periodEnd")]
    pub period_end: Option<String>,
    #[serde(rename = "submitDateTime")]
    submit_date_time: Option<String>,
    #[serde(rename = "docDescription")]
    pub doc_description: Option<String>,
    #[serde(rename = "issuerEdinetCode")]
    issuer_edinet_code: Option<String>,
    #[serde(rename = "subjectEdinetCode")]
    subject_edinet_code: Option<String>,
    #[serde(rename = "subsidiaryEdinetCode")]
    subsidiary_edinet_code: Option<String>,
    #[serde(rename = "currentReportReason")]
    current_report_reason: Option<String>,
    #[serde(rename = "parentDocID")]
    parent_doc_id: Option<String>,
    #[serde(rename = "opeDateTime")]
    ope_date_time: Option<String>,
    #[serde(rename = "withdrawalStatus")]
    withdrawal_status: String,
    #[serde(rename = "docInfoEditStatus")]
    doc_info_edit_status: String,
    #[serde(rename = "disclosureStatus")]
    disclosure_status: String,
    #[serde(rename = "xbrlFlag")]
    xbrl_flag: String,
    #[serde(rename = "pdfFlag")]
    pdf_flag: String,
    #[serde(rename = "attachDocFlag")]
    attach_doc_flag: String,
    #[serde(rename = "englishDocFlag")]
    english_doc_flag: String,
}

impl DisclosureResult {
    pub fn filter_ufo(&self) -> bool {
        let sec_code_existing: bool = self.sec_code.is_some();
        let ufo_flag: bool = if let Some(ufo_flag) = &self.doc_type_code { ufo_flag == "120" } else { false };
        sec_code_existing && ufo_flag
    }
}
