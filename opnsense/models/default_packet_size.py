from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class DefaultPacketSize(BaseModel):
    class Meta:
        name = "defaultPacketSize"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
