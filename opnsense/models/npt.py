from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Npt(BaseModel):
    class Meta:
        name = "npt"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
