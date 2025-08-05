from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class TriggerInitialWizard(BaseModel):
    class Meta:
        name = "trigger_initial_wizard"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
