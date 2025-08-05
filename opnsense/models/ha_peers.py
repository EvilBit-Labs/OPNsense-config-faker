from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class HaPeers(BaseModel):
    class Meta:
        name = "ha_peers"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
