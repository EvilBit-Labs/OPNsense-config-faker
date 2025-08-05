from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class EventqueueSlots(BaseModel):
    class Meta:
        name = "eventqueueSlots"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
