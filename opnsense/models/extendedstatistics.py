from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Extendedstatistics(BaseModel):
    class Meta:
        name = "extendedstatistics"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
