from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Send(BaseModel):
    class Meta:
        name = "send"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
