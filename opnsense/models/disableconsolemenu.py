from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Disableconsolemenu(BaseModel):
    class Meta:
        name = "disableconsolemenu"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
