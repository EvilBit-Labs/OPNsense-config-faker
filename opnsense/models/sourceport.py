from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Sourceport(BaseModel):
    class Meta:
        name = "sourceport"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
