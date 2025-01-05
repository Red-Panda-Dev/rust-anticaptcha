use std::fmt;

pub trait TaskTypeTrait {
    fn as_string(&self) -> String;
}

#[derive(Debug)]
pub enum ImageTaskType {
    ImageToTextTask,
    ImageToCoordinatesTask,
}
impl TaskTypeTrait for ImageTaskType {
    fn as_string(&self) -> String {
        format!("{}", &self)
    }
}

#[derive(Debug)]
pub enum TokenTaskType {
    RecaptchaV2Task,
    RecaptchaV2TaskProxyless,

    RecaptchaV3Enterprise,
    RecaptchaV3TaskProxyless,

    RecaptchaV2EnterpriseTask,
    RecaptchaV2EnterpriseTaskProxyless,

    FunCaptchaTask,
    FunCaptchaTaskProxyless,

    GeeTestTask,
    GeeTestTaskProxyless,

    TurnstileTask,
    TurnstileTaskProxyless,

    AntiGateTask,
}
impl TaskTypeTrait for TokenTaskType {
    fn as_string(&self) -> String {
        format!("{}", &self)
    }
}

#[derive(Debug)]
pub enum EnpPostfix {
    // tasks processing
    createTask,
    getTaskResult,
    // get account info
    getBalance,
    getQueueStats,
    getAppStats,
    getSpendingStats,
    // reports
    reportIncorrectImageCaptcha,
    reportIncorrectRecaptcha,
    reportCorrectRecaptcha,
    reportIncorrectHcaptcha,
    pushAntiGateVariable,
}
impl fmt::Display for EnpPostfix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum GetResultStatus {
    processing,
    ready,
    error,
}
impl fmt::Display for GetResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
