from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Txtsupport(BaseModel):
    class Meta:
        name = "txtsupport"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
