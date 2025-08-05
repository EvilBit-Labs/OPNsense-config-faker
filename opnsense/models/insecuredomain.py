from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Insecuredomain(BaseModel):
    class Meta:
        name = "insecuredomain"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
