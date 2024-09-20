use crate::RUDIS_DB;
use resp::Value;
use std::io;

pub fn process_client_request(decoded_msg: Value) -> Result<Vec<u8>, io::Error> {
    let reply = if let Value::Array(v) = decoded_msg {
        match &v[0] {
            Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
            Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
            other => handle_other(other),
        } 
    } else {
        Err(Value::Error("Invalid Command".to_string()))
    };

    // Map the Value::Error into an io::Error
    match reply {
        Ok(r) => Ok(r.encode()),  // Success case
        Err(r) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", r))), // Convert custom error to io::Error
    }
}

pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() {
        return Err(Value::Error("Expected 1 argument for GET command".to_string()))
    }
    
    let db_ref = RUDIS_DB.lock().unwrap();
    
    let reply = if let Value::Bulk(ref s) = &v[0] {
        db_ref.get(s).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
    } else {
        Value::Null
    };

    Ok(reply)
}

pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error("Expected 2 argument for GET command".to_string()))
    }
        
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            let _ = RUDIS_DB.lock().unwrap().insert(k.to_string(), v.to_string());
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
    } 

    Ok(Value::String("OK".to_string()))
}

pub fn handle_other(_v: &Value) -> Result<Value, Value> {
    Err(Value::Error("Method not supported as of now".to_string()))
}