use bytes::{ Bytes, BytesMut, BufMut };

pub fn generate_response(val: Bytes, erorr_code: u16) -> crate::Result<Bytes> {
    /* Generate status code and header for the response. */
    let resp_str = 
        format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n", erorr_code, val.len());
    
    /* Now add the actual response body. */
    let resp_bin = resp_str.as_bytes();
    let mut response = BytesMut::with_capacity(resp_bin.len() + val.len() + 5);
    response.put(resp_str.as_bytes());
    response.put(val);
    Ok(response.freeze())
}
