use persistence_std::{types::{cosmwasm::wasm::v1::MsgExecuteContract, cosmos::gov::v1::{QueryProposalsResponse, QueryProposalRequest, QueryProposalsRequest, ProposalStatus}}, shim::Any};
use serde::de::Error;


#[test]
pub fn json_deserialization_fix_test() {
    let json = r#"{"@type":"/cosmwasm.wasm.v1.MsgExecuteContract","sender":"persistence10d07y265gmmuvt4z0w9aw880jnsr700j5w4kch","contract":"persistence1xxx3ps3gm3wceg4g300hvggdv7ga0hmsk64srccffmfy4wvcrugq3skeny","msg":{"resume_create_pool":{"pool_creation_request_id":1}},"funds":[]}"#;

    let test: Any = serde_json_wasm::from_str(json).unwrap();

    // convert any back to msg execute contract
    let msg: MsgExecuteContract = MsgExecuteContract::try_from(test).unwrap();

    // get msg of msg to validate the json deserialization
    let str: String = String::from_utf8(msg.msg.clone()).unwrap();
    assert_eq!(str, r#"{"resume_create_pool":{"pool_creation_request_id":1}}"#);

    // str.as_bytes().to_vec();

}

#[test]
pub fn enum_serde_test() {
    let query = QueryProposalsRequest {
        proposal_status: ProposalStatus::DepositPeriod.into(),
        voter: "".to_string(),
        depositor: "".to_string(),
        pagination: None,
    };

    let json = serde_json_wasm::to_string(&query).unwrap();
    println!("{}", json);
    let query2: QueryProposalsRequest = serde_json_wasm::from_str(&json).unwrap();

    assert_eq!(query, query2);


    let test_json = r#"{"proposals":[{"id":"3","messages":[{"@type":"/cosmwasm.wasm.v1.MsgExecuteContract","sender":"persistence10d07y265gmmuvt4z0w9aw880jnsr700j5w4kch","contract":"persistence1xxx3ps3gm3wceg4g300hvggdv7ga0hmsk64srccffmfy4wvcrugq3skeny","msg":{"resume_create_pool":{"pool_creation_request_id":2}},"funds":[]}],"status":"PROPOSAL_STATUS_DEPOSIT_PERIOD","final_tally_result":{"yes_count":"0","abstain_count":"0","no_count":"0","no_with_veto_count":"0"},"submit_time":"2023-09-06T16:32:52.516964Z","deposit_end_time":"2023-09-08T16:32:52.516964Z","total_deposit":[],"voting_start_time":null,"voting_end_time":null,"metadata":"test","title":"Test Proposal","summary":"test","proposer":"persistence1xxx3ps3gm3wceg4g300hvggdv7ga0hmsk64srccffmfy4wvcrugq3skeny"}],"pagination":{"next_key":null,"total":"1"}}"#;
    let val: Result<QueryProposalsResponse, serde_json_wasm::de::Error> = serde_json_wasm::from_str(test_json);

    println!("{:?}", val);

}