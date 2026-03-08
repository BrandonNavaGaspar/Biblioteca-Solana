use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tienda_telefonos {
    use super::*;

    //////////////////////////// Crear Tienda ////////////////////////////
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let telefonos: Vec<Telefono> = Vec::new();

        context.accounts.tienda.set_inner(TiendaTelefonos {
            owner: owner_id,
            nombre,
            telefonos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Telefono ////////////////////////////
    pub fn agregar_telefono(
        context: Context<NuevoTelefono>,
        nombre: String,
        marca: String,
        precio: u32
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let telefono = Telefono {
            nombre,
            marca,
            precio,
            disponible: true,
        };

        context.accounts.tienda.telefonos.push(telefono);

        Ok(())
    }

    //////////////////////////// Eliminar Telefono ////////////////////////////
    pub fn eliminar_telefono(context: Context<NuevoTelefono>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let telefonos = &mut context.accounts.tienda.telefonos;

        for i in 0..telefonos.len() {

            if telefonos[i].nombre == nombre {

                telefonos.remove(i);
                msg!("Telefono {} eliminado!", nombre);

                return Ok(());

            }

        }

        Err(Errores::TelefonoNoExiste.into())
    }

    //////////////////////////// Ver Telefonos ////////////////////////////
    pub fn ver_telefonos(context: Context<NuevoTelefono>) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista de telefonos: {:#?}",
            context.accounts.tienda.telefonos
        );

        Ok(())
    }

    //////////////////////////// Alternar Disponibilidad ////////////////////////////
    pub fn alternar_estado(context: Context<NuevoTelefono>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let telefonos = &mut context.accounts.tienda.telefonos;

        for i in 0..telefonos.len() {

            let estado = telefonos[i].disponible;

            if telefonos[i].nombre == nombre {

                let nuevo_estado = !estado;

                telefonos[i].disponible = nuevo_estado;

                msg!(
                    "El telefono {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());

            }

        }

        Err(Errores::TelefonoNoExiste.into())
    }
}

//////////////////////////// Errores ////////////////////////////

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("Error, el telefono no existe")]
    TelefonoNoExiste,

}

//////////////////////////// Cuenta Tienda ////////////////////////////

#[account]
#[derive(InitSpace)]

pub struct TiendaTelefonos {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    telefonos: Vec<Telefono>,

}

//////////////////////////// Struct Telefono ////////////////////////////

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]

pub struct Telefono {

    #[max_len(60)]
    nombre: String,

    #[max_len(40)]
    marca: String,

    precio: u32,

    disponible: bool,

}

//////////////////////////// Contexto Crear Tienda ////////////////////////////

#[derive(Accounts)]

pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = TiendaTelefonos::INIT_SPACE + 8,
        seeds = [b"tienda_telefonos", owner.key().as_ref()],
        bump
    )]

    pub tienda: Account<'info, TiendaTelefonos>,

    pub system_program: Program<'info, System>,

}

//////////////////////////// Contexto Telefonos ////////////////////////////

#[derive(Accounts)]

pub struct NuevoTelefono<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, TiendaTelefonos>,

}
