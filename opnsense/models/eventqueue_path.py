from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class EventqueuePath(BaseModel):
    class Meta:
        name = "eventqueuePath"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
