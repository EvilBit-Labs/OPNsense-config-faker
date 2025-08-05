from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Psk(BaseModel):
    class Meta:
        name = "psk"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
