from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Infracachenumhosts(BaseModel):
    class Meta:
        name = "infracachenumhosts"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
