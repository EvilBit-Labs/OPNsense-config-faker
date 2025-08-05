from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Mirror(BaseModel):
    class Meta:
        name = "mirror"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
