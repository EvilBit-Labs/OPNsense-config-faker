from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Synchronizetoip(BaseModel):
    class Meta:
        name = "synchronizetoip"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
