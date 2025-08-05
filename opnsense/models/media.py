from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Media(BaseModel):
    class Meta:
        name = "media"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
