from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Infrahostttl(BaseModel):
    class Meta:
        name = "infrahostttl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
