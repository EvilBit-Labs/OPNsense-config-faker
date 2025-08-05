from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class OutgoingInterface(BaseModel):
    class Meta:
        name = "outgoing_interface"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
