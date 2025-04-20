use pushx::{DingTalkClient, PushClient, PushMessage};

#[tokio::main]
async fn main() {
    let client = DingTalkClient::new(
        "https://oapi.dingtalk.com/robot/send?access_token=".to_string(),
        None,
    );
    let message = PushMessage {
        title: "提醒".into(),
        content: "当前任务已完成".into(),
    };

    if let Err(e) = client.send(message).await {
        eprintln!("推送失败: {}", e);
    }
}
