from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Verbosity(BaseModel):
    class Meta:
        name = "verbosity"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
