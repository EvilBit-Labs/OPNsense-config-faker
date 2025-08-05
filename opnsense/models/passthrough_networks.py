from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class PassthroughNetworks(BaseModel):
    class Meta:
        name = "passthrough_networks"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
