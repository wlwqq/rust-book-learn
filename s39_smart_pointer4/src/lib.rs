pub trait Messenger {
    fn send(&self, msg: &str);
}

// 当value到达max时，就调用messager发送一条通知消息
pub struct LimitTracker<'a, T: Messenger> {
    messager: &'a T,
    value: i64,
    max: i64,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messager: &T, max: i64) -> LimitTracker<T> {
        LimitTracker {
            messager: messager,
            value: 0,
            max: max,
        }
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 下面会报错，因为&self获取的是不可变引用，所以不能对结构体做修改操作
            // self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
