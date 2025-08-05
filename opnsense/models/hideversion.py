from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Hideversion(BaseModel):
    class Meta:
        name = "hideversion"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
