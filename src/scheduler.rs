use chrono::{self, Local};
use windows::core::{BSTR, Result, Interface};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance, COINIT_MULTITHREADED, CoInitializeEx};
use windows::Win32::System::TaskScheduler::{
    IAction, IActionCollection, IDailyTrigger, IExecAction, IPrincipal,
    ITaskDefinition, ITaskFolder, ITaskService, ITaskSettings, ITrigger,
    ITriggerCollection, TASK_ACTION_EXEC, TASK_CREATE_OR_UPDATE,
    TASK_LOGON_INTERACTIVE_TOKEN, TASK_RUNLEVEL_LUA, TASK_TRIGGER_DAILY,
    TaskScheduler
};

pub fn schedule(action_path: String) -> Result<()> {
    let task_service = get_task_service()?;

    unsafe {
        let task_definition: ITaskDefinition = task_service.NewTask(0)?;

        let actions: IActionCollection = task_definition.Actions()?;
        let action: IAction = actions.Create(TASK_ACTION_EXEC)?;
        let action: IExecAction = action.cast()?;

        action.SetId(&BSTR::from("action"))?;
        action.SetPath(&BSTR::from(action_path))?;

        let triggers: ITriggerCollection = task_definition.Triggers()?;
        let trigger: ITrigger = triggers.Create(TASK_TRIGGER_DAILY)?;
        let trigger: IDailyTrigger = trigger.cast()?;

        trigger.SetId(&BSTR::from("time_trigger"))?;
        trigger.SetStartBoundary(&BSTR::from((Local::now() + chrono::Duration::minutes(1)).format("%Y-%m-%dT%H:%M:%S%:z").to_string()))?;
        trigger.SetEnabled(true as i16)?;

        let principal: IPrincipal = task_definition.Principal()?;
        principal.SetRunLevel(TASK_RUNLEVEL_LUA)?;

        let folder: ITaskFolder = task_service.GetFolder(&BSTR::from(r"\"))?;

        let settings: ITaskSettings = task_definition.Settings()?;
        settings.SetHidden(false as i16)?;
        settings.SetEnabled(true as i16)?;

        folder.RegisterTaskDefinition(
            &BSTR::from("distorter"),
            &task_definition,
            TASK_CREATE_OR_UPDATE.0,
            None,
            None,
            TASK_LOGON_INTERACTIVE_TOKEN,
            None,
        )?;
    }

    Ok(())
}

fn get_task_service() -> Result<ITaskService> {
    unsafe {
        CoInitializeEx(Some(std::ptr::null_mut()), COINIT_MULTITHREADED)?;

        let task_service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;
        task_service.Connect(None, None, None, None)?;

        Ok(task_service)
    }
}