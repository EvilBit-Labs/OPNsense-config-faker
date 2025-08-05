from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Stop(BaseModel):
    class Meta:
        name = "stop"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
