from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Spoofmac(BaseModel):
    class Meta:
        name = "spoofmac"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
