extern crate mini_functions_date;
extern crate mini_functions_jwt_claims;

use self::mini_functions_date::Date;
use self::mini_functions_jwt_claims::Claims;

fn main() {
    const CL_AUD: &str = "MINI-FUNCTIONS-CLAIMS-AUD";
    const CL_CUSTOM: &str = "MINI-FUNCTIONS-CLAIMS-CUSTOM";
    const CL_DID: &str = "MINI-FUNCTIONS-CLAIMS-DID";
    const CL_ISS: &str = "MINI-FUNCTIONS-CLAIMS-ISS";
    const CL_JTI: &str = "MINI-FUNCTIONS-CLAIMS-JTI";
    const CL_SUB: &str = "MINI-FUNCTIONS-CLAIMS-SUB";
    const CL_VC: &str = "MINI-FUNCTIONS-CLAIMS-VC";
    const CL_VP: &str = "MINI-FUNCTIONS-CLAIMS-VP";

    let date = Date::new().iso_8601;
    let claims = Claims {
        aud: Some(CL_AUD.to_string()),
        custom: Some(CL_CUSTOM.to_string()),
        did: Some(CL_DID.to_string()),
        exp: Some(date.read().unwrap().to_string()),
        iat: Some(date.read().unwrap().to_string()),
        iss: Some(CL_ISS.to_string()),
        jti: Some(CL_JTI.to_string()),
        nbf: Some(date.read().unwrap().to_string()),
        sub: Some(CL_SUB.to_string()),
        vc: Some(CL_VC.to_string()),
        vp: Some(CL_VP.to_string()),
    };
    println!("{}", claims);
}
