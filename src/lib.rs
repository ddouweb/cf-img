use worker::*;
//use reqwest::Client;
#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let url = req.url()?;
    let path = url.path();
    // 如果请求路径匹配 /file/*
    if path.starts_with("/file/") {
        let uuid = &path["/file/".len()..]; // 提取 UUID
        return handle_image_request(uuid, &env).await;
    }
    // 如果路径不匹配 /file/*，返回 404
    Response::error("Not Found", 404)
}

async fn handle_image_request(uuid: &str, env: &Env) -> Result<Response> {
    let kv = env.kv("redis")?;
    if let Some(original_url) = kv.get(uuid).text().await? {
        // 使用内置 fetch API 进行请求
        let request = Request::new(&original_url, Method::Get)?;
        let mut response = worker::Fetch::Request(request).send().await?;
        //let body = response.bytes().await?;
        let mut headers = Headers::new();
        headers.append("Content-Type", "image/jpeg")?;

        return Response::from_bytes(response.bytes().await?)
            .map(|res| res.with_headers(headers));
    }
    Response::error("Image not found", 404)
}

// async fn handle_image_request1(uuid: &str, env: &worker::Env) -> worker::Result<worker::Response> {
//     let kv = env.kv("redis")?;
//     //kv.put(uuid, "https://alipicdeco.ys7.com:8089/ezviz/pic/down?url=aHR0cDovL2hpay1hbGljbG91ZC5vc3MtY24taGFuZ3pob3UtaW50ZXJuYWwuYWxpeXVuY3MuY29tLzcvQkM1NTUxMDQ5XzFfMjAyNDA4MTUxODMxMjAtQkM1NTUxMDQ5LTEtMTUwMTAtMi0xP0V4cGlyZXM9MTcyNDMyMjY4NyZPU1NBY2Nlc3NLZXlJZD1MVEFJNEdHcnExYTlrRndVNEJtR1pkblQmU2lnbmF0dXJlPVNTRWhaUDkzdFp2aDdoOFRWUDBBdzRZJTJGejdnJTNE&crypt=2&time=2024-08-15T18:31:19&key=0a7a106974b996066e3df137c0ac5b96")?.execute().await?;
//     // 从 KV 中获取 URL
//     let original_url = match kv.get(uuid).text().await {
//         Ok(Some(url)) => url,
//         Ok(None) => return Response::error("Image not found", 404),
//         Err(e) => return Response::error(&format!("Failed to retrieve URL from KV: {:?}", e), 500),
//     };

//     console_log!("Received request: 获取url:{}",original_url);
//     let client = Client::new();
//     // 发起 HTTP 请求
//     let response = match client
//             .get(&original_url)
//             //.timeout(Duration::from_secs(3)) // 设置超时时间为3秒
//             .send()
//             .await 
//         {
//             Ok(resp) => resp,
//             Err(e) => return Response::error(format!("Request failed: {:?}", e), 500),
//         };

//     console_log!("收到请求！！！：{:?}",response);

//      // 处理响应体
//      let body = match response.bytes().await {
//         Ok(bytes) => bytes.to_vec(),
//         Err(e) => return Response::error(&format!("Failed to read response body: {:?}", e), 500),
//     };

//     // 设置响应头
//     let mut headers = Headers::new();
//     headers.append("Content-Type", "image/jpeg")?;

//     // 构建和返回响应
//     Response::from_bytes(body)
//         .map(|res| res.with_headers(headers))
// }