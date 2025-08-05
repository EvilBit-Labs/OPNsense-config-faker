from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Reminder(BaseModel):
    class Meta:
        name = "reminder"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
