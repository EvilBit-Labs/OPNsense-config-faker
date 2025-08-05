from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Reboot(BaseModel):
    class Meta:
        name = "reboot"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
