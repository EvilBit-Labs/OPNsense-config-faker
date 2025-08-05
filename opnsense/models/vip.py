from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Vip(BaseModel):
    class Meta:
        name = "vip"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
