from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Privatedomain(BaseModel):
    class Meta:
        name = "privatedomain"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
