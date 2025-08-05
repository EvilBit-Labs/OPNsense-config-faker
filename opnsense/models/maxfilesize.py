from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Maxfilesize(BaseModel):
    class Meta:
        name = "maxfilesize"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
