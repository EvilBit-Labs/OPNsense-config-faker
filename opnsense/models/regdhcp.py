from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Regdhcp(BaseModel):
    class Meta:
        name = "regdhcp"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
