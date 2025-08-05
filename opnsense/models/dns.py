from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dns(BaseModel):
    class Meta:
        name = "dns"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
