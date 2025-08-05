from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Vlan(BaseModel):
    class Meta:
        name = "vlan"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
