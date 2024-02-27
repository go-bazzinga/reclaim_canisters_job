pub use console_error_panic_hook::set_once as set_panic_hook;
use worker::*;

// #[event(fetch)]
// async fn main(_req: worker::Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
//
#[event(scheduled)]
pub async fn scheduled_event(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_log!("Hello from a scheduled event!");
    set_panic_hook();

    let url = "https://icp-off-chain-agent.fly.dev/start_backup";

    let client = reqwest::Client::new();
    let res = match client
        .get(url)
        .bearer_auth(
            env.secret("CF_WORKER_ACCESS_OFF_CHAIN_AGENT_KEY")
                .expect("CF_WORKER_ACCESS_OFF_CHAIN_AGENT_KEY not found")
                .to_string(),
        )
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            console_log!("Error: {:?}", err);
            return;
        }
    };

    console_log!("Response: {:?}", res);
    let body = match res.text().await {
        Ok(body) => body,
        Err(err) => {
            console_log!("Error: {:?}", err);
            return;
        }
    };
    console_log!("Body: {:?}", body);
}
