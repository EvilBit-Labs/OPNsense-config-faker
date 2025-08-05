from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Mediaopt(BaseModel):
    class Meta:
        name = "mediaopt"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
