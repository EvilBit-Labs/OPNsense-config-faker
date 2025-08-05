from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Ntpserver(BaseModel):
    class Meta:
        name = "ntpserver"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
