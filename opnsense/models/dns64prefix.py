from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dns64Prefix(BaseModel):
    class Meta:
        name = "dns64prefix"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
