from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AliasAddress(BaseModel):
    class Meta:
        name = "alias-address"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
