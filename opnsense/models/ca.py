from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Ca(BaseModel):
    class Meta:
        name = "ca"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
