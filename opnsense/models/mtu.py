from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Mtu(BaseModel):
    class Meta:
        name = "mtu"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
