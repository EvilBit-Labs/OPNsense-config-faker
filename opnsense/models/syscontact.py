from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Syscontact(BaseModel):
    class Meta:
        name = "syscontact"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
