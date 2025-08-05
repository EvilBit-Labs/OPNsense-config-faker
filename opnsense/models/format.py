from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Format(BaseModel):
    class Meta:
        name = "format"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
