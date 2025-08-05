from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class ActiveInterface(BaseModel):
    class Meta:
        name = "active_interface"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
